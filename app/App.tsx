import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';
import Game from './Game';

export default function App() {
  const endpoint = "http://127.0.0.1:8899";
  const wallets = [new PhantomWalletAdapter()];

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <Game />
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}