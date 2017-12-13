
import styled, { css } from 'styled-components'
import * as theme from 'theme'

const headerStyles = css`
  margin: 0;
  padding: 0;
  display: inline-block;
`
export const MiniHeader = styled.span`
  font-size: ${theme.miniHeaderFontSize};
  line-height: ${theme.miniHeaderLineHeight};
  color: ${theme.lesserFontColor};
  text-transform: uppercase;
`
export const H1 = styled.h1`
  ${headerStyles}
  font-size: ${theme.h1FontSize}
  font-weight: ${theme.h1FontWeight};
  line-height: ${theme.h1LineHeight};
`
export const H2 = styled.h2`
  ${headerStyles}
  font-size: ${theme.h2FontSize}
  font-weight: ${theme.h2FontWeight};
  line-height: ${theme.h2LineHeight};
`

export const H3 = styled.h3`
  ${headerStyles}
  font-size: ${theme.h3FontSize}
  font-weight: ${theme.h3FontWeight};
  line-height: ${theme.h3LineHeight};
`

export const H4 = styled.h4`
  ${headerStyles}
  font-size: ${theme.h4FontSize}
  font-weight: ${theme.h4FontWeight};
  line-height: ${theme.h4LineHeight};
`

export const P = styled.h4`
  margin: 0 0 ${theme.gutterSmall} 0;
  font-size: ${theme.fontSize}
  line-height: ${theme.lineHeight};
`
