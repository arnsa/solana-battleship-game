const webpack = require('webpack');
const paths = require('react-scripts/config/paths');
const path = require('path');
const ModuleScopePlugin = require('react-dev-utils/ModuleScopePlugin');

paths.appSrc = path.resolve(__dirname, 'app')
paths.appIndexJs = path.resolve(__dirname, 'app/index.tsx')

module.exports = function override(config) {
  const fallback = config.resolve.fallback || {};


  config.resolve.plugins = config.resolve.plugins.filter(plugin => !(plugin instanceof ModuleScopePlugin));
  Object.assign(fallback, {
    "crypto": require.resolve("crypto-browserify"),
    "stream": require.resolve("stream-browserify"),
    "http": require.resolve("stream-http"),
    "https": require.resolve("https-browserify"),
    "zlib": require.resolve("browserify-zlib"),
    "url": require.resolve("url/")
  });
  config.resolve.fallback = fallback;
  config.plugins = (config.plugins || []).concat([
    new webpack.ProvidePlugin({
      process: 'process/browser',
      Buffer: ['buffer', 'Buffer']
    })
  ]);
  return config;
};