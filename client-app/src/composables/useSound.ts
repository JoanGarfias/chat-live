export function useSound (){
    const playNotificationSound = () => {
        const audio = new Audio('notification.mp3');
        audio.play().catch((error) => {
            console.error("Error al reproducir el sonido de notificación:", error);
        });
    }

    return {
        playNotificationSound
    }
}