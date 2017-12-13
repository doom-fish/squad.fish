//@flow

type WallabyType = {
    compilers: {
        babel: () => mixed
    },
    testFramework: {
        configure: () => mixed
    },
    localProjectDir: string
};
const WallabyConfig = (wallaby: WallabyType) => ({
    files: ['src/**/*.js', '!src/**/*.test.js', '!node_modules/**/*.js'],

    tests: ['src/**/*.test.js', '!node_modules/**/*.js'],

    env: {
        type: 'node',
        runner: '/Users/perjohansson/.config/fnm/bin/node',
        params: {
            runner: '--harmony'
        }
    },
    compilers: {
        '**/*.js': wallaby.compilers.babel()
    },
    setup: function(wallaby: WallabyType) {
        wallaby.testFramework.configure({
            moduleFileExtensions: ['jsx', 'js', 'json'],
            moduleNameMapper: {
                '^.+\\.(jpg|jpeg|png|gif|eot|otf|webp|svg|ttf|woff|woff2|mp4|webm|wav|mp3|m4a|aac|oga)$': wallaby.localProjectDir +
                    '/config/jest/FileStub.js',
                '^.+\\.css$': wallaby.localProjectDir +
                    '/config/jest/CSSStub.js'
            },
            setupFiles: [wallaby.localProjectDir + '/config/polyfills.js'],
            testPathIgnorePatterns: [
                wallaby.localProjectDir + '/(build|docs|node_modules)/'
            ],
            testEnvironment: 'node'
        })
    },
    testFramework: 'jest',
    debug: true
})

module.exports = WallabyConfig
