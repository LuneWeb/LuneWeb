return {
	gui = [[
        let __ELEMENTS = [];

        function e(tag, innerHtml) {
            const el = document.createElement(tag);

            el.innerHTML = innerHtml;

            return __ELEMENTS.push(el) - 1;
        }
        
        function attach(i) {
            const el = __ELEMENTS[i];
            document.body.append(el);
        }
        ]],
}
