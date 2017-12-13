import React from 'react';

export default function spectrum(props) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      xmlnsXlink="http://www.w3.org/1999/xlink"
      width={722}
      height={142}
      viewBox="0 0 361 71"
      {...props}
    >
      <defs>
        <style
          dangerouslySetInnerHTML={{
            __html: `.cls-1 {
                stroke-linejoin: round;
                stroke-width: 1px;
                font-size: 52px;
                stroke: url(#linear-gradient);
                font-family: "Proxima Nova";
                font-weight: 700;
              }`,
          }}
        />
        <linearGradient
          id="linear-gradient"
          x1="91.242"
          y1={4}
          x2="252.758"
          y2={66}
          gradientUnits="userSpaceOnUse"
        >
          <stop offset="0%" stopColor="#00c3ff" />
          <stop offset="100%" stopColor="#ffff1c" />
        </linearGradient>
      </defs>
      <text id="spectrum" textAnchor="middle" className="cls-1" y="50%" x="50%">
        spectrum
      </text>
    </svg>
  );
}
