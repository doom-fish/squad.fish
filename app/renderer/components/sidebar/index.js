
import React from 'react'
import styled from 'styled-components'
import { MiniHeader } from '../typography'
import { panelBackground } from 'theme'
const SideBar = styled.div`
  grid-area: sidebar;
  flex-direction: column;
  display:flex;

  background-color: ${panelBackground};
  justify-content: : center;
`
export default () => (
    <SideBar>
        <MiniHeader>menu</MiniHeader>
        <div>ae</div>
    </SideBar>
)
