/**
 * @cluaiz/extension-my-assistant
 * Zero-VRAM interactive canvas desktop companion extension for Cluaiz Engine
 */

export { AssistantCanvas, AssistantExpression, AssistantTheme, AssistantOptions } from './AssistantCanvas';
export { AssistantVoiceSync, VoiceSyncState, MouthShape } from './AssistantVoiceSync';

import { AssistantCanvas, AssistantOptions } from './AssistantCanvas';

/**
 * Initializes and mounts the companion avatar onto the DOM
 */
export function initMyAssistant(options?: AssistantOptions): AssistantCanvas | null {
    if (typeof document === 'undefined') return null;
    return new AssistantCanvas(options);
}
