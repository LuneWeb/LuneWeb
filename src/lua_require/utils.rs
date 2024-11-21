use crate::LuaAppProxyMethods;
use mlua::prelude::*;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone)]
enum ModuleReturn {
    Running(flume::Receiver<mlua::Result<mlua::MultiValue>>),
    Finished(mlua::Result<mlua::MultiValue>),
}

#[derive(Debug, Clone)]
struct ModuleCache(HashMap<PathBuf, ModuleReturn>);

fn load_cache(lua: &mlua::Lua) -> mlua::AppDataRef<ModuleCache> {
    if let Some(cache) = lua.app_data_ref::<ModuleCache>() {
        cache
    } else {
        // lazy load module cache
        let this = ModuleCache(Default::default());
        lua.set_app_data(this);
        lua.app_data_ref::<ModuleCache>().unwrap()
    }
}

pub async fn load_script(lua: mlua::Lua, path: PathBuf) -> LuaResult<LuaMultiValue> {
    let contents = smol::fs::read_to_string(&path)
        .await
        .map_err(|x| match x.kind() {
            std::io::ErrorKind::NotFound => mlua::Error::runtime(format!(
                "The system cannot find {} (os error 2)",
                path.to_string_lossy()
            )),
            _ => x.into_lua_err(),
        })?;

    let chunk = lua.load(contents).set_name(path.to_string_lossy());
    let thread = lua.create_thread(chunk.into_function()?)?;
    let proxy = lua.get_app_proxy();

    proxy.spawn_lua_thread(thread.clone(), None);
    proxy.await_lua_thread(thread).await
}

pub async fn load_module(lua: mlua::Lua, path: PathBuf) -> LuaResult<LuaMultiValue> {
    let cached_module = {
        let cache = load_cache(&lua);
        cache.0.get(&path).map(|x| x.to_owned())
    };

    if let Some(module) = cached_module {
        match module {
            ModuleReturn::Running(channel) => {
                return channel.recv_async().await.into_lua_err()?;
            }
            ModuleReturn::Finished(result) => {
                return result.to_owned();
            }
        }
    }

    let cache_sender = {
        let (sender, receiver) = flume::unbounded();

        let mut cache = lua.app_data_mut::<ModuleCache>().unwrap();
        cache
            .0
            .insert(path.clone(), ModuleReturn::Running(receiver));

        sender
    };

    let result = load_script(lua.clone(), path.clone()).await;

    {
        cache_sender
            .send_async(result.clone())
            .await
            .into_lua_err()?;

        let mut cache = lua.app_data_mut::<ModuleCache>().unwrap();
        cache
            .0
            .insert(path.clone(), ModuleReturn::Finished(result.clone()))
    };

    result
}
