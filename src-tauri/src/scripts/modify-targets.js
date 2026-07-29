(function hookLinkTargets() {
    console.info('[uXueScript] 注入高效链接劫持...');

    // 1. 劫持全页面的点击事件，防止 target="_blank" 或 _top 唤起外部默认浏览器
    document.addEventListener('click', function (e) {
        const anchor = e.target.closest('a[href]');
        if (anchor) {
            const target = anchor.getAttribute('target');
            if (target === '_blank' || target === '_top') {
                console.info('[uXueScript] 拦截弹窗链接，强制在当前 Webview 原位加载:', anchor.href);
                anchor.setAttribute('target', '_self'); // 强制在当前 Webview 原位打开
            }
        }
    }, true);

    // 2. 劫持 setAttribute 方法，阻止动态设置 _blank 或 _top
    const originalSetAttribute = Element.prototype.setAttribute;
    Element.prototype.setAttribute = function (name, value) {
        if (this instanceof HTMLAnchorElement && name.toLowerCase() === 'target') {
            if (value === '_blank' || value === '_top') {
                value = '_self'; // 强行重定向为 _self
            }
        }
        return originalSetAttribute.call(this, name, value);
    };

    // 3. 劫持 window.open，强行在当前 Webview 内直接原位加载
    const originalOpen = window.open;
    window.open = function (url, target, features) {
        if (url) {
            console.info('[uXueScript] 劫持 window.open，当前 Webview 原位跳转:', url);
            window.location.href = url; // 强行在当前 Webview 内直接加载
            return window;
        }
        return originalOpen.call(this, url, target, features);
    };
})();
