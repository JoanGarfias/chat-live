import type { KeyboardEvent } from '@/types/KeyboardEvent';

export const addKeyboardListener = (e: KeyboardEvent, callback: () => void, key: string) => {
    if(e.key === key) {
        callback();
    }
}