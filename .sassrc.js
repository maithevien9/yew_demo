const path = require('path');
// const ReactIcons = require('react-icons');

const CWD = process.cwd();

module.exports = {
  includePaths: [path.resolve(CWD, 'node_modules'), path.resolve(CWD, 'src')],
};
