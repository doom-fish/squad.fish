const arrayRemove = require('unordered-array-remove');
const http = require('http');
const mime = require('mime');
const pump = require('pump');
const rangeParser = require('range-parser');
const url = require('url');
const serverDestroy = require('server-destroy');
module.exports = file => {
  const server = http.createServer();
  serverDestroy(server);
  const onRequest = (req, res) => {
    var pathname = url.parse(req.url).pathname;

    // Allow CORS requests to read responses
    if (req.headers.origin) {
      res.setHeader('Access-Control-Allow-Origin', req.headers.origin || '*');
    }
    if (pathname === '/favicon.ico') {
      return serve404Page();
    }
    // Prevent browser mime-type sniffing
    res.setHeader('X-Content-Type-Options', 'nosniff');

    // Allow CORS requests to specify arbitrary headers, e.g. 'Range',
    // by responding to the OPTIONS preflight request with the specified
    // origin and requested headers.
    if (req.method === 'OPTIONS') {
      return serveOptionsRequest();
    }

    res.statusCode = 200;
    res.setHeader('Content-Type', mime.lookup(file.name));

    // Support range-requests
    res.setHeader('Accept-Ranges', 'bytes');

    // Set name of file (for "Save Page As..." dialog)
    res.setHeader(
      'Content-Disposition',
      "inline; filename*=UTF-8''" + encodeRFC5987(file.name)
    );

    // Support DLNA streaming
    res.setHeader('transferMode.dlna.org', 'Streaming');
    res.setHeader(
      'contentFeatures.dlna.org',
      'DLNA.ORG_OP=01;DLNA.ORG_CI=0;DLNA.ORG_FLAGS=01700000000000000000000000000000'
    );

    // `rangeParser` returns an array of ranges, or an error code (number) if
    // there was an error parsing the range.
    var range = rangeParser(file.length, req.headers.range || '');
    if (Array.isArray(range)) {
      res.statusCode = 206; // indicates that range-request was understood

      // no support for multi-range request, just use the first range
      range = range[0];

      res.setHeader(
        'Content-Range',
        'bytes ' + range.start + '-' + range.end + '/' + file.length
      );
      res.setHeader('Content-Length', range.end - range.start + 1);
    } else {
      range = null;
      res.setHeader('Content-Length', file.length);
    }

    if (req.method === 'HEAD') {
      return res.end();
    }

    pump(file.createReadStream(range), res);

    const serveOptionsRequest = () => {
      res.statusCode = 204; // no content
      res.setHeader('Access-Control-Max-Age', '600');
      res.setHeader(
        'Access-Control-Allow-Methods',
        'GET,HEAD,PUT,PATCH,POST,DELETE'
      );

      if (req.headers['access-control-request-headers']) {
        res.setHeader(
          'Access-Control-Allow-Headers',
          req.headers['access-control-request-headers']
        );
      }
      res.end();
    };
  };

  server.on('request', onRequest);

  return server;
};

// From https://developer.mozilla.org/en/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent
const encodeRFC5987 = str =>
  encodeURIComponent(str)
    // Note that although RFC3986 reserves "!", RFC5987 does not,
    // so we do not need to escape it
    .replace(/['()]/g, escape) // i.e., %27 %28 %29
    .replace(/\*/g, '%2A')
    // The following are not required for percent-encoding per RFC5987,
    // so we can allow for a little better readability over the wire: |`^
    .replace(/%(?:7C|60|5E)/g, unescape);
