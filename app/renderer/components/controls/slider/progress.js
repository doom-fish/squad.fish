//@flow
import React from 'react'
import styled from 'styled-components'
import * as theme from 'theme'

const Progress = styled.div`
    height: 0.5rem;
    border-radius: 5px;
    background-color: ${theme.progressColor};
    transition: width ${({ dragging }) => (dragging ? 0 : '100ms')} linear;
`

type ProgressPropTypes = {
    position: number,
    dragging: boolean,
}

export default ({ position, dragging }: ProgressPropTypes) => (
    <Progress dragging={dragging} style={{ width: `${position}%` }} />
)
