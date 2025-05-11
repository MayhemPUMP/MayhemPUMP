import React from 'react';
import styled from '@emotion/styled';

interface CardProps {
  title?: string;
  image?: string;
  rarity?: 'common' | 'rare' | 'epic' | 'legendary';
  onClick?: () => void;
  children: React.ReactNode;
}

const StyledCard = styled.div<Pick<CardProps, 'rarity'>>`
  background: #1A1A1A;
  border-radius: 16px;
  padding: 20px;
  width: 100%;
  max-width: 320px;
  cursor: pointer;
  transition: transform 0.2s ease;
  
  ${props => {
    switch (props.rarity) {
      case 'legendary':
        return `
          border: 2px solid #FFD700;
          box-shadow: 0 0 20px rgba(255, 215, 0, 0.2);
        `;
      case 'epic':
        return `
          border: 2px solid #9B59B6;
          box-shadow: 0 0 20px rgba(155, 89, 182, 0.2);
        `;
      case 'rare':
        return `
          border: 2px solid #3498DB;
          box-shadow: 0 0 20px rgba(52, 152, 219, 0.2);
        `;
      default:
        return `
          border: 2px solid #95A5A6;
          box-shadow: 0 0 20px rgba(149, 165, 166, 0.2);
        `;
    }
  }}

  &:hover {
    transform: translateY(-5px);
  }
`;

const CardImage = styled.img`
  width: 100%;
  height: 200px;
  object-fit: cover;
  border-radius: 8px;
  margin-bottom: 16px;
`;

const CardTitle = styled.h3`
  color: #FFFFFF;
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 12px 0;
`;

const CardContent = styled.div`
  color: #A0AEC0;
`;

export const Card: React.FC<CardProps> = ({
  title,
  image,
  rarity = 'common',
  onClick,
  children,
}) => {
  return (
    <StyledCard rarity={rarity} onClick={onClick}>
      {image && <CardImage src={image} alt={title || 'Card image'} />}
      {title && <CardTitle>{title}</CardTitle>}
      <CardContent>{children}</CardContent>
    </StyledCard>
  );
};