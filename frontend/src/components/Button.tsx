import React from 'react';
import styled from '@emotion/styled';

interface ButtonProps {
  variant?: 'primary' | 'secondary' | 'outline';
  size?: 'small' | 'medium' | 'large';
  disabled?: boolean;
  onClick?: () => void;
  children: React.ReactNode;
}

const StyledButton = styled.button<ButtonProps>`
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  
  ${props => {
    switch (props.variant) {
      case 'secondary':
        return `
          background: #2A2A2A;
          color: #FFFFFF;
          border: none;
          &:hover {
            background: #404040;
          }
        `;
      case 'outline':
        return `
          background: transparent;
          color: #7C3AED;
          border: 2px solid #7C3AED;
          &:hover {
            background: rgba(124, 58, 237, 0.1);
          }
        `;
      default:
        return `
          background: #7C3AED;
          color: #FFFFFF;
          border: none;
          &:hover {
            background: #6D28D9;
          }
        `;
    }
  }}

  ${props => {
    switch (props.size) {
      case 'small':
        return `
          padding: 8px 16px;
          font-size: 14px;
        `;
      case 'large':
        return `
          padding: 16px 32px;
          font-size: 18px;
        `;
      default:
        return `
          padding: 12px 24px;
          font-size: 16px;
        `;
    }
  }}

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    &:hover {
      background: ${props => props.variant === 'primary' ? '#7C3AED' : 
        props.variant === 'secondary' ? '#2A2A2A' : 'transparent'};
    }
  }
`;

export const Button: React.FC<ButtonProps> = ({
  variant = 'primary',
  size = 'medium',
  disabled = false,
  onClick,
  children,
}) => {
  return (
    <StyledButton
      variant={variant}
      size={size}
      disabled={disabled}
      onClick={onClick}
    >
      {children}
    </StyledButton>
  );
};