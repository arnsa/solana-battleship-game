import { useState } from 'react';
import { Connection, PublicKey } from '@solana/web3.js';
import { Program, AnchorProvider, web3 } from "@coral-xyz/anchor";
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import idl from "../target/idl/solana_battleship_game.json";

const connection = new Connection("http://127.0.0.1:8899", 'confirmed');

export default function Game() {
  const { publicKey } = useWallet();
  const [status, setStatus] = useState('');

  const getProgram = () => {
    const provider = new AnchorProvider(
      connection,
      window.solana,
      { commitment: 'confirmed' }
    );
    return new Program(idl, provider);
  };

  const initializeGame = async () => {
    if (!publicKey) {
      setStatus('Please connect your wallet first');
      return;
    }

    try {
      setStatus('Initializing game...');
      const program = getProgram();

      const [gameAccount] = await PublicKey.findProgramAddressSync(
        [Buffer.from("battleship-game"), publicKey.toBuffer()],
        program.programId
      );

      const shipCoordinates = [
        {
          col: 0,
          row: 0,
          direction: { down: null }
        },
        {
          col: 1,
          row: 0,
          direction: { down: null }
        },
        {
          col: 2,
          row: 0,
          direction: { down: null }
        },
        {
          col: 3,
          row: 0,
          direction: { down: null }
        },
        {
          col: 4,
          row: 0,
          direction: { down: null }
        },
      ];

      const modifyComputeUnits = web3.ComputeBudgetProgram.setComputeUnitLimit({
        units: 1000000
      });
      const tx = await program.methods
        .initialize(shipCoordinates)
        .accounts({
          gameAccount,
          player: publicKey,
          systemProgram: web3.SystemProgram.programId,
        })
        .preInstructions([modifyComputeUnits])
        .rpc();

      setStatus(`Game initialized! Transaction: ${tx}`);
    } catch (error) {
      setStatus(`Error: ${error.message}`);
      console.error('Error:', error);
      error.getLogs?.().then(console.log);
    }
  };

  const closeGame = async () => {
    if (!publicKey) {
      setStatus('Please connect your wallet first');
      return;
    }

    try {
      setStatus('Closing game...');
      const program = getProgram();

      const [gameAccount] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("battleship-game"),
          publicKey.toBuffer(),
        ],
        program.programId,
      );

      const tx = await program.methods
        .closeGame()
        .accounts({
          gameAccount,
          payer: publicKey,
        })
        .rpc();

      setStatus(`Game account closed! Transaction: ${tx}`);
    } catch (error: any) {
      setStatus(`Error: ${error.message}`);
      console.error('Error:', error);
      if (error.logs) {
        console.log(error.logs);
      }
    }
  };

  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">Solana Battleship Game</h1>
      <WalletMultiButton className="mb-4" />
      <button
        onClick={initializeGame}
        disabled={!publicKey}
        className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50"
      >
        Initialize Game
      </button>
      <button
        onClick={closeGame}
        disabled={!publicKey}
        className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50"
      >
        Close Game
      </button>
      <div className="mt-4">
        <p className="text-gray-700">Status: {status}</p>
      </div>
    </div>
  );
}