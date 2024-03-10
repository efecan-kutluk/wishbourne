import { WalletUser } from "@/components/shared/interfaces";

export class AuthService{
    currentUser?:WalletUser;

    async connectWallet(){

    }

    async signUp(walletPublicKey:string|null){
        this.currentUser={
            Email:"",
            Name:"",
            Lastname:"",
            Wallet:walletPublicKey??""
        }
    }
}