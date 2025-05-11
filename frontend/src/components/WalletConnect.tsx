import React from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import styled from '@emotion/styled';
import { Button } from './Button';

const WalletWrapper = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
`;

const WalletStatus = styled.div<{ isConnected: boolean }>`
  display: flex;
  align-items: center;
  gap: 8px;
  color: ${props => props.isConnected ? '#10B981' : '#EF4444'};
  font-size: 14px;

  &::before {
    content: '';
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: ${props => props.isConnected ? '#10B981' : '#EF4444'};
  }
`;

const WalletBalance = styled.div`
  color: #FFFFFF;
  font-size: 16px;
  font-weight: 600;
`;

const WalletAddress = styled.div`
  color: #A0AEC0;
  font-size: 14px;
`;

export const WalletConnect: React.FC = () => {
  const { connected, publicKey, disconnect } = useWallet();

  const shortenAddress = (address: string) => {
    return `${address.slice(0, 4)}...${address.slice(-4)}`;
  };

  return (
    <WalletWrapper>
      <WalletStatus isConnected={connected}>
        {connected ? '已连接' : '未连接'}
      </WalletStatus>

      {connected && publicKey ? (
        <>
          <WalletBalance>SOL 余额: 0.00</WalletBalance>
          <WalletAddress>
            {shortenAddress(publicKey.toString())}
          </WalletAddress>
          <Button
            variant="outline"
            size="small"
            onClick={disconnect}
          >
            断开连接
          </Button>
        </>
      ) : (
        <WalletMultiButton />
      )}
    </WalletWrapper>
  );
};