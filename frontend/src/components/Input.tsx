import React from 'react';
import styled from '@emotion/styled';

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
  icon?: React.ReactNode;
}

const InputWrapper = styled.div`
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
`;

const InputLabel = styled.label`
  color: #FFFFFF;
  font-size: 14px;
  font-weight: 500;
`;

const InputContainer = styled.div`
  position: relative;
  display: flex;
  align-items: center;
`;

const StyledInput = styled.input<{ hasError?: boolean }>`
  width: 100%;
  padding: 12px 16px;
  background: #2A2A2A;
  border: 2px solid ${props => props.hasError ? '#EF4444' : '#4A5568'};
  border-radius: 8px;
  color: #FFFFFF;
  font-size: 16px;
  transition: all 0.2s ease;

  &:focus {
    outline: none;
    border-color: ${props => props.hasError ? '#EF4444' : '#7C3AED'};
    box-shadow: 0 0 0 3px ${props => props.hasError ? 'rgba(239, 68, 68, 0.2)' : 'rgba(124, 58, 237, 0.2)'};
  }

  &::placeholder {
    color: #718096;
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
`;

const IconWrapper = styled.div`
  position: absolute;
  right: 16px;
  color: #718096;
`;

const ErrorMessage = styled.span`
  color: #EF4444;
  font-size: 14px;
`;

export const Input: React.FC<InputProps> = ({
  label,
  error,
  icon,
  ...props
}) => {
  return (
    <InputWrapper>
      {label && <InputLabel>{label}</InputLabel>}
      <InputContainer>
        <StyledInput hasError={!!error} {...props} />
        {icon && <IconWrapper>{icon}</IconWrapper>}
      </InputContainer>
      {error && <ErrorMessage>{error}</ErrorMessage>}
    </InputWrapper>
  );
};