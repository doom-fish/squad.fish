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
        <g
            fill='#FFF'
        >
            <path
                d='M16 17.031a1 1 0 0 0 1-1v-8a1 1 0 1 0-2 0v8a1 1 0 0 0 1 1z'
            />
            <path
                d='M19 9.618v2.223a5.993 5.993 0 0 1 3 5.19 6 6 0 0 1-12 0 5.992 5.992 0 0 1 3-5.19V9.618a7.999 7.999 0 0 0 3 15.413 7.999 7.999 0 1 0 3-15.413z'
            />
        </g>
    </svg>
)
