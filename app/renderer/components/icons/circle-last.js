import React from 'react'

export default props => (
    <svg
        version='1'
        viewBox='0 0 32 32'
        {...props}
    >
        <circle
            cx='16'
            cy='16'
            r='16'
            fill='#469FCC'
        />
        <path
            d='M16 32c8.837 0 16-7.163 16-16S24.837 0 16 0 0 7.163 0 16s7.163 16 16 16zm0-30c7.732 0 14 6.268 14 14s-6.268 14-14 14S2 23.732 2 16 8.268 2 16 2z'
            opacity='.15'
        />
        <g
            fill='#FFF'
        >
            <path
                d='M11.99 11.031c-.952-.709-1.219-1-2-1-1.008 0-1 1-1 1v10s0 1 1 1c.734 0 1.062-.309 2-1l5.531-4.016c.625-.422.656-1.375-.016-1.984l-5.515-4zM21.99 10.031h-2a1 1 0 0 0-1 1v10a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-10a1 1 0 0 0-1-1z'
            />
        </g>
    </svg>
)
