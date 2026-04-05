(function() {
    console.log('注入渐变透明脚本...');
    
    const style = document.createElement('style');
    style.textContent = `
        @keyframes fadeOutPage {
            from { opacity: 1; }
            to { opacity: 0; }
        }
        html.fading-out,
        html.fading-out body {
            animation: fadeOutPage 500ms cubic-bezier(0.42, 0, 0.58, 1) forwards !important;
            background-color: transparent !important;
            overflow: hidden !important;
        }
    `;
    document.head.appendChild(style);
    document.documentElement.classList.add('fading-out');
    
})();