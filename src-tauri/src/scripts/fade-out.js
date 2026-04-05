(function() {
    console.log('注入渐变透明脚本...');
    
    globalThis.fadeOutPage = function(duration = 500, easing = 'cubic-bezier(0.42, 0, 0.58, 1)') {
        const style = document.createElement('style');
        style.textContent = `
            @keyframes fadeOutPage {
                from { opacity: 1; }
                to { opacity: 0; }
            }
            html.fading-out,
            html.fading-out body {
                animation: fadeOutPage ${duration}ms ${easing} forwards !important;
                background-color: transparent !important;
                overflow: hidden !important;
            }
        `;
        document.head.appendChild(style);
        document.documentElement.classList.add('fading-out');
    };
    
    console.log('fadeOutPage 函数已注入');
    globalThis.fadeOutPage();
})();