import React from 'react'

export default props => (
    <svg
        version='1'
        viewBox='0 0 32 32'
        {...props}
    >
        <circle
            cx='16'
            cy='16.031'
            r='16'
            fill='#469FCC'
        />
        <path
            d='M16 .031c-8.837 0-16 7.163-16 16s7.163 16 16 16 16-7.163 16-16-7.163-16-16-16zm0 30c-7.732 0-14-6.268-14-14s6.268-14 14-14 14 6.268 14 14-6.268 14-14 14z'
            opacity='.15'
        />
        <g
            fill='#FFF'
        >
            <path
                d='M13 9.031h-2a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-12a1 1 0 0 0-1-1zM21 9.031h-2a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-12a1 1 0 0 0-1-1z'
            />
        </g>
    </svg>
)
