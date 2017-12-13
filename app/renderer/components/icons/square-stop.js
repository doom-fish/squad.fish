import React from 'react'

export default props => (
    <svg
        version='1'
        viewBox='0 0 32 32'
        {...props}
    >
        <path
            fill='#469FCC'
            d='M32 30.031a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2v-28a2 2 0 0 1 2-2h28a2 2 0 0 1 2 2v28z'
        />
        <path
            d='M30 .031H2a2 2 0 0 0-2 2v28a2 2 0 0 0 2 2h28a2 2 0 0 0 2-2v-28a2 2 0 0 0-2-2zm0 30H2v-28h28v28z'
            opacity='.15'
        />
        <path
            fill='#FFF'
            d='M22 21.062a1 1 0 0 1-1 1H11a1 1 0 0 1-1-1v-10a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v10z'
        />
    </svg>
)
