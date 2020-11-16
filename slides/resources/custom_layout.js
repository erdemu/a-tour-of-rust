import React, { Fragment } from "react";
import { useDeck } from "mdx-deck"; // Import useDeck
import styled from "@emotion/styled";

const Wrapper = styled.main`
  align-items: center;
  display: flex;
  flex-direction: row;
  height: 100vh;
  justify-content: center;
  position: relative;
  width: 100vw;
  & > div[class*="Split"] {
    height: 90vh;
  }
  & h1 {
    padding-top: 2rem;
  }
  & > div > div:first-child {
    height: 90vh !important;
  }
`;

const Footer = styled.footer`
  border-top: 1px solid ${(props) => props.color};
  color: black;
  display: flex;
  justify-content: space-between;
  padding: 30px 100px;
  text-align: right;
  width: 100vw;
`;

const SlideFooter = ({ conf, speaker, children, color = "#ebfbfc" }) => {
  const state = useDeck(); // Declare a new state variable

  const currentSlide = state.index + 1; // The slides are zero-index

  return (
    <Fragment>
      <Wrapper>{children}</Wrapper>
      <Footer color={color}>
        <span>{conf}</span>
        <span>{speaker}</span>
        <span>{currentSlide}</span>
      </Footer>
    </Fragment>
  );
};

export default SlideFooter;
