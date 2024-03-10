import { reactive } from "vue";
import { AuthService } from "./services/AuthService";

const authService = reactive(new AuthService());
export function useGlobalState() {
    return {
        authService
    };
}