export function useChat(){

    const getState = () => {
        return "connected";
    }

    return {
        getState
    }
}