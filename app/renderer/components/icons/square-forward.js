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
            d='M24.516 15.062l-5.516-4c-.952-.709-1.219-1-2-1-1.008 0-1 1-1 1v2.9l-4-2.9c-.952-.709-1.219-1-2-1-1.008 0-1 1-1 1v10s0 1 1 1c.734 0 1.062-.309 2-1l4-2.904v2.904s0 1 1 1c.734 0 1.062-.309 2-1l5.531-4.016c.625-.421.657-1.374-.015-1.984z'
        />
    </svg>
)
