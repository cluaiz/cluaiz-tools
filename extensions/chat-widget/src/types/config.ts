export interface StudioConfig {
    theme: {
        primaryColor: string;
        secondaryColor: string;
        useGradient: boolean;
        iconColor: string;
        iconStyle: 'solid' | 'outline';
        showBox: boolean;
        boxRadius: number;
        iconScale: number;
        boxAnimation: string;
    };
    icon: {
        type: string;
        size: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
    };
    animation: {
        robot: string;
        simple: string;
        entrance: string;
        speed: number;
        intensity: number;
    };
    position: {
        horizontal: 'left' | 'right';
        vertical: 'bottom' | 'top';
        offsetX: number;
        offsetY: number;
    };
    behavior: {
        robot: {
            hover: string;
            click: string;
        };
        icon: {
            hover: string;
            click: string;
        };
    };
    robot: {
        eyeColor: string;
        cheekColor: string;
        lipColor: string;
        earColor: string;
        eyebrowColor: string;
        bodyColor: string;
        useGradient: boolean;
        gradientType: string;
        gradientColor1: string;
        gradientColor2: string;
        gradientAngle: number;
        animateGradient: boolean;
    };
    window: {
        theme: 'auto' | 'light' | 'dark';
        colorMode: 'auto' | 'custom';
        customColor?: string;
    };
}

export interface WidgetConfig {
    brandConfig?: any;
    homeConfig?: any;
    securityConfig?: any;
    widgetConfig?: StudioConfig;
}
