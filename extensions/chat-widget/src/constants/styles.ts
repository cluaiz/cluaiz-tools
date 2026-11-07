export const WIDGET_STYLES = `
    @keyframes bot-blink {
        0%, 90%, 100% { transform: scaleY(1); }
        92% { transform: scaleY(0.1); }
        94% { transform: scaleY(1); }
        96% { transform: scaleY(0.1); }
        98% { transform: scaleY(1); }
    }
    .cluaiz-eye-blink { animation: bot-blink 5s infinite; }

    #cluaiz-launcher {
        position: fixed; bottom: 20px; right: 20px; overflow: visible;
        width: 100px; height: 100px; cursor: pointer; z-index: 2147483647;
        user-select: none; transform-origin: center center; transition: transform 0.3s ease;
        --body-color: #FFFFFF; --eye-color: #3B82F6; --lip-color: #F472B6;
        --primary-color: #3B82F6; --secondary-color: #8B5CF6;
        --anim-speed: 1.0; --anim-intensity: 1.0;
        --gradient-color1: #3B82F6; --gradient-color2: #8B5CF6; --gradient-angle: 45deg;
    }
    .cluaiz-box {
        width: 100%; height: 100%; position: relative;
        display: flex; align-items: center; justify-content: center;
        background: var(--box-bg, linear-gradient(135deg, var(--gradient-color1), var(--gradient-color2)));
        border-radius: var(--box-radius, 50%);
        box-shadow: var(--box-shadow, 0 8px 32px rgba(0,0,0,0.15));
        border: var(--box-border, 1px solid rgba(255,255,255,0.15));
    }
    .cluaiz-layer {
        position: absolute; top: 50%; left: 50%;
        transform: translate(calc(-50% + var(--offset-x, 0px)), calc(-50% + var(--offset-y, 0px)));
        transition: transform 0.1s ease-out; will-change: transform; pointer-events: none;
    }
    .cluaiz-pupil {
        transform: translate(var(--pupil-x, 0px), var(--pupil-y, 0px));
        transition: transform 0.1s cubic-bezier(0.25, 1.6, 0.5, 1); transform-origin: center;
    }

    .bot-container-anim { width: 100%; height: 100%; animation: bot-idle 4s ease-in-out infinite; }
    @keyframes bot-idle { 0%, 100% { transform: translateY(0) scale(1); } 50% { transform: translateY(-3px) scale(1.02); } }
    @keyframes bot-nod {
        0%, 100% { transform: translateY(0) rotate(0deg); }
        50% { transform: translateY(8px) rotate(-5deg); }
    }
    @keyframes robot-float {
        0%, 100% { transform: translateY(0px) rotate(0deg); }
        25% { transform: translateY(calc(-5px * var(--anim-intensity, 1))) rotate(calc(-1deg * var(--anim-intensity, 1))); }
        50% { transform: translateY(calc(-8px * var(--anim-intensity, 1))) rotate(0deg); }
        75% { transform: translateY(calc(-5px * var(--anim-intensity, 1))) rotate(calc(1deg * var(--anim-intensity, 1))); }
    }
    @keyframes robot-wave {
        0%, 100% { transform: rotate(0deg); }
        10%, 30% { transform: rotate(calc(-8deg * var(--anim-intensity, 1))); }
        20%, 40% { transform: rotate(calc(8deg * var(--anim-intensity, 1))); }
        50% { transform: rotate(0deg); }
    }
    @keyframes robot-bounce {
        0%, 100% { transform: translateY(0) scaleY(1); }
        10% { transform: translateY(0) scaleY(1.1) scaleX(0.9); }
        30% { transform: translateY(calc(-20px * var(--anim-intensity, 1))) scaleY(1.15) scaleX(0.95); }
        50% { transform: translateY(0) scaleY(0.9) scaleX(1.05); }
        57% { transform: translateY(calc(-7px * var(--anim-intensity, 1))) scaleY(1.05) scaleX(0.95); }
        64% { transform: translateY(0) scaleY(1); }
    }
    @keyframes robot-peep {
        0%, 100% { transform: rotate(0deg) translateX(0); }
        25% { transform: rotate(calc(-15deg * var(--anim-intensity, 1))) translateX(calc(-5px * var(--anim-intensity, 1))); }
        35% { transform: rotate(calc(-15deg * var(--anim-intensity, 1))) translateX(calc(-5px * var(--anim-intensity, 1))); }
        40% { transform: rotate(0deg) translateX(0); }
    }
    @keyframes robot-glitch {
        0%, 100% { transform: translate(0, 0); filter: none; }
        10% { transform: translate(calc(-2px * var(--anim-intensity, 1)), calc(2px * var(--anim-intensity, 1))); filter: hue-rotate(90deg); }
        20% { transform: translate(calc(2px * var(--anim-intensity, 1)), calc(-2px * var(--anim-intensity, 1))); filter: hue-rotate(-90deg); }
        30% { transform: translate(0, 0); filter: none; }
    }
    @keyframes robot-spin {
        0%, 100% { transform: rotate(0deg) translateY(0); }
        25% { transform: rotate(calc(-12deg * var(--anim-intensity, 1))) translateY(calc(-4px * var(--anim-intensity, 1))); }
        75% { transform: rotate(calc(12deg * var(--anim-intensity, 1))) translateY(calc(-4px * var(--anim-intensity, 1))); }
    }
    @keyframes box-ripple {
        0% { box-shadow: 0 0 0 0 var(--secondary-color), 0 0 0 0 var(--primary-color); }
        40% { box-shadow: 0 0 0 calc(15px * var(--anim-intensity, 1)) rgba(255,255,255,0), 0 0 0 0 var(--primary-color); }
        80% { box-shadow: 0 0 0 calc(15px * var(--anim-intensity, 1)) rgba(255,255,255,0), 0 0 0 calc(30px * var(--anim-intensity, 1)) rgba(255,255,255,0); }
        100% { box-shadow: 0 0 0 0 rgba(255,255,255,0), 0 0 0 calc(30px * var(--anim-intensity, 1)) rgba(255,255,255,0); }
    }
    @keyframes icon-ripple {
        0% { transform: scale(1); filter: drop-shadow(0 0 0 rgba(59,130,246,0.4)); }
        50% { transform: scale(1.05); filter: drop-shadow(0 0 15px var(--primary-color)); }
        100% { transform: scale(1); filter: drop-shadow(0 0 0 rgba(59,130,246,0.4)); }
    }
    @keyframes box-glow {
        0%, 100% { box-shadow: 0 8px 32px rgba(0,0,0,0.15), 0 0 10px var(--primary-color, #3B82F6); }
        50% { box-shadow: 0 8px 32px rgba(0,0,0,0.25), 0 0 25px var(--primary-color, #3B82F6); }
    }
    @keyframes icon-glow {
        0%, 100% { filter: drop-shadow(0 0 2px var(--primary-color, #3B82F6)) drop-shadow(0 0 4px var(--primary-color, #3B82F6)); opacity: 1; }
        50% { filter: drop-shadow(0 0 8px var(--primary-color, #3B82F6)) drop-shadow(0 0 16px var(--primary-color, #3B82F6)); opacity: 1; }
    }
    @keyframes icon-float {
        0%, 100% { transform: translateY(0px) scale(var(--icon-scale, 1)); }
        50% { transform: translateY(-4px) scale(var(--icon-scale, 1)); }
    }
    @keyframes icon-pulse {
        0%, 100% { transform: scale(var(--icon-scale, 1)); filter: brightness(100%); }
        50% { transform: scale(calc(var(--icon-scale, 1) * 1.1)); filter: brightness(120%); }
    }
    @keyframes box-morph {
        0%, 100% { border-radius: var(--box-radius, 50%); }
        25% { border-radius: 40% 60% 60% 40% / 60% 30% 70% 40%; }
        50% { border-radius: 60% 40% 30% 70% / 60% 30% 70% 40%; }
        75% { border-radius: 40% 60% 60% 40% / 30% 60% 40% 70%; }
    }
    @keyframes icon-morph {
        0%, 100% { transform: scale(var(--icon-scale, 1)) rotate(0deg); }
        33% { transform: scale(calc(var(--icon-scale, 1) * 1.05)) rotate(2deg); }
        66% { transform: scale(calc(var(--icon-scale, 1) * 0.95)) rotate(-2deg); }
    }
    @keyframes icon-shake {
        0%, 100% { transform: scale(var(--icon-scale, 1)) translateX(0); }
        10%, 30%, 50%, 70%, 90% { transform: scale(var(--icon-scale, 1)) translateX(-3px); }
        20%, 40%, 60%, 80% { transform: scale(var(--icon-scale, 1)) translateX(3px); }
    }
    @keyframes icon-bounce {
        0%, 100% { transform: scale(var(--icon-scale, 1)) translateY(0); }
        50% { transform: scale(var(--icon-scale, 1)) translateY(calc(-10px * var(--anim-intensity, 1))); }
    }
    @keyframes icon-swing {
        20% { transform: rotate(15deg); } 40% { transform: rotate(-10deg); }
        60% { transform: rotate(5deg); } 80% { transform: rotate(-5deg); }
        0%, 100% { transform: rotate(0deg); }
    }
    @keyframes icon-beat {
        0%, 100% { transform: scale(var(--icon-scale, 1)); }
        14% { transform: scale(calc(var(--icon-scale, 1) * 1.3)); }
        28% { transform: scale(var(--icon-scale, 1)); }
        42% { transform: scale(calc(var(--icon-scale, 1) * 1.3)); }
        70% { transform: scale(var(--icon-scale, 1)); }
    }
    @keyframes icon-tilt {
        0%, 100% { transform: perspective(100px) rotateX(0deg) rotateY(0deg); }
        25% { transform: perspective(100px) rotateX(5deg) rotateY(5deg); }
        50% { transform: perspective(100px) rotateX(-5deg) rotateY(5deg); }
        75% { transform: perspective(100px) rotateX(-5deg) rotateY(-5deg); }
    }

    .robot-float { animation: robot-float calc(4s / var(--anim-speed)) ease-in-out infinite; }
    .robot-wave { animation: robot-wave calc(3s / var(--anim-speed)) ease-in-out infinite; }
    .robot-bounce { animation: robot-bounce calc(1.5s / var(--anim-speed)) ease-in-out infinite; }
    .robot-peep { animation: robot-peep calc(5s / var(--anim-speed)) ease-in-out infinite; }
    .robot-glitch { animation: robot-glitch calc(3s / var(--anim-speed)) steps(1) infinite; }
    .robot-spin { animation: robot-spin calc(2s / var(--anim-speed)) ease-in-out infinite; }
    .bot-nod { animation: bot-nod 0.6s ease-in-out; }

    .cluaiz-icon-animator { transform-origin: center center; will-change: transform; width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; }
    .cluaiz-icon-float { transform-origin: center center; will-change: transform; transform: scale(var(--icon-scale, 1)); transition: all 0.3s ease; }

    .box-anim-ripple .cluaiz-box { animation: box-ripple calc(4s / var(--anim-speed)) cubic-bezier(0, 0, 0.2, 1) infinite; }
    .box-anim-pulse .cluaiz-box { animation: icon-pulse calc(4s / var(--anim-speed)) ease-in-out infinite; }
    .box-anim-morph .cluaiz-box { animation: box-morph calc(10s / var(--anim-speed)) ease-in-out infinite; }
    .box-anim-glow .cluaiz-box { animation: box-glow calc(5s / var(--anim-speed)) ease-in-out infinite; }
    .box-anim-shake .cluaiz-box { animation: icon-shake calc(4s / var(--anim-speed)) ease-in-out infinite; }
    .box-anim-bounce .cluaiz-box { animation: icon-bounce calc(2s / var(--anim-speed)) ease-in-out infinite; }
    .box-anim-swing .cluaiz-box { animation: icon-swing calc(4s / var(--anim-speed)) ease-in-out infinite; }
    .box-anim-beat .cluaiz-box { animation: icon-beat calc(3s / var(--anim-speed)) ease-in-out infinite; }
    .box-anim-tilt .cluaiz-box { animation: icon-tilt calc(6s / var(--anim-speed)) ease-in-out infinite; }

    .icon-anim-float .cluaiz-icon-animator { animation: icon-float 3s ease-in-out infinite; }
    .icon-anim-ripple .cluaiz-icon-animator { animation: icon-ripple calc(2s / var(--anim-speed)) ease-out infinite; }
    .icon-anim-pulse .cluaiz-icon-animator { animation: icon-pulse calc(1.5s / var(--anim-speed)) ease-in-out infinite; }
    .icon-anim-morph .cluaiz-icon-animator { animation: icon-morph calc(3s / var(--anim-speed)) ease-in-out infinite; }
    .icon-anim-shake .cluaiz-icon-animator { animation: icon-shake calc(2s / var(--anim-speed)) ease-in-out infinite; }
    .icon-anim-bounce .cluaiz-icon-animator { animation: icon-bounce calc(0.8s / var(--anim-speed)) ease-in-out infinite; }
    .icon-anim-swing .cluaiz-icon-animator { animation: icon-swing calc(2s / var(--anim-speed)) ease-in-out infinite; }
    .icon-anim-beat .cluaiz-icon-animator { animation: icon-beat calc(1.5s / var(--anim-speed)) ease-in-out infinite; }
    .icon-anim-glow .cluaiz-icon-animator { animation: icon-glow calc(2s / var(--anim-speed)) ease-in-out infinite; }
    .icon-anim-tilt .cluaiz-icon-animator { animation: icon-tilt calc(3s / var(--anim-speed)) ease-in-out infinite; }

    #cluaiz-launcher.is-hovered { transform: scale(1.05); transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1); }
    #cluaiz-launcher.hover-effect-scale.is-hovered { transform: scale(1.15); }
    #cluaiz-launcher.hover-effect-tilt.is-hovered { transform: rotate(15deg) scale(1.1); }
    #cluaiz-launcher.is-active { transform: scale(0.95); transition: transform 0.1s ease-out; }
    #cluaiz-launcher.has-notification { animation: attention-pulse 1.5s ease-in-out infinite; }
    #cluaiz-launcher.cluaiz-dragging { cursor: grabbing !important; z-index: 2147483648; opacity: 0.8; }

    @keyframes attention-pulse { 0%, 100% { transform: scale(1); } 50% { transform: scale(1.05); } }

    .notification-badge {
        position: absolute; top: -5px; right: -5px; min-width: 20px; height: 20px; padding: 0 6px;
        background: #EF4444; border-radius: 10px; font-size: 11px; font-weight: 600; color: white;
        display: flex; align-items: center; justify-content: center;
        box-shadow: 0 2px 8px rgba(239, 68, 68, 0.6);
        animation: badge-pop 0.3s cubic-bezier(0.68, -0.55, 0.265, 1.55); z-index: 10;
    }
    @keyframes badge-pop { 0% { transform: scale(0); opacity: 0; } 50% { transform: scale(1.2); } 100% { transform: scale(1); opacity: 1; } }

    @keyframes entrance-popup {
        0% { transform: translateY(100px) scale(0); opacity: 0; }
        60% { transform: translateY(-10px) scale(1.1); }
        80% { transform: translateY(5px) scale(0.95); }
        100% { transform: translateY(0) scale(1); opacity: 1; }
    }
    @keyframes entrance-slide { 0% { transform: translateX(100px); opacity: 0; } 100% { transform: translateX(0); opacity: 1; } }
    @keyframes entrance-fade { 0% { opacity: 0; } 100% { opacity: 1; } }
    @keyframes entrance-zoom { 0% { transform: scale(0); opacity: 0; } 100% { transform: scale(1); opacity: 1; } }
    @keyframes entrance-rotate { 0% { transform: scale(0) rotate(-180deg); opacity: 0; } 100% { transform: scale(1) rotate(0deg); opacity: 1; } }
    @keyframes entrance-elastic {
        0% { transform: translateY(100px) scale(0.5); opacity: 0; }
        40% { transform: translateY(-20px) scale(1.1); }
        60% { transform: translateY(10px) scale(0.95); }
        80% { transform: translateY(-5px) scale(1.05); }
        100% { transform: translateY(0) scale(1); opacity: 1; }
    }
    .entrance-popup { animation: entrance-popup 0.6s cubic-bezier(0.68, -0.55, 0.265, 1.55); }
    .entrance-slide { animation: entrance-slide 0.5s ease-out; }
    .entrance-fade { animation: entrance-fade 0.8s ease-in; }
    .entrance-zoom { animation: entrance-zoom 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275); }
    .entrance-rotate { animation: entrance-rotate 0.6s cubic-bezier(0.175, 0.885, 0.32, 1.275); }
    .entrance-elastic { animation: entrance-elastic 0.8s cubic-bezier(0.68, -0.55, 0.265, 1.55); }

    @keyframes gradient-pulse { 0% { filter: brightness(1); } 50% { filter: brightness(1.1) saturate(1.2); } 100% { filter: brightness(1); } }
    .gradient-animated .robot-body { animation: gradient-pulse 2s ease-in-out infinite; }

    /* ── Chat Iframe / Modal ── */
    #cluaiz-iframe {
        position: fixed; width: 380px; height: 600px;
        border: none; border-radius: 18px; box-shadow: 0 20px 50px rgba(0,0,0,0.2);
        z-index: 2147483646; opacity: 0; pointer-events: none;
        transform: translateY(20px) scale(0.95);
        transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    }
    #cluaiz-iframe.cluaiz-visible { opacity: 1; pointer-events: all; transform: translateY(0) scale(1); }
    #cluaiz-iframe.cluaiz-sidebar-mode {
        transform: none !important; box-shadow: -5px 0 30px rgba(0,0,0,0.1);
        transition: width 0.3s cubic-bezier(0.16, 1, 0.3, 1), left 0.3s cubic-bezier(0.16, 1, 0.3, 1), right 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    }

    /* ── Resizer ── */
    #cluaiz-resizer {
        position: fixed; width: 10px; height: 100vh;
        z-index: 2147483647; background: transparent; transition: background 0.2s;
        display: none; align-items: center; justify-content: center; cursor: col-resize;
    }
    #cluaiz-resizer.active { display: flex; }
    #cluaiz-resizer:hover { background: rgba(59, 130, 246, 0.3); }

    @media (max-width: 480px) {
        #cluaiz-iframe { width: 92%; height: 75%; }
        #cluaiz-launcher { width: 80px; height: 80px; }
    }
`;
