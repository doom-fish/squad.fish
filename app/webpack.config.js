//@flow
const WebpackConfig = () => {
    if (process.env.NODE_ENV === 'development') {
        return require('./config/webpack.dev.config.js')
    } else {
        return require('./config/webpack.prod.config.js')
    }
}

module.exports = WebpackConfig
