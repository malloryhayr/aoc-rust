// https://github.com/vitejs/vite/discussions/8553#discussioncomment-4200201

import { writeFile } from 'fs/promises';
import packageJson from './pkg/package.json' assert { type: 'json' };

packageJson.main = packageJson.module;
packageJson.type = 'module';
delete packageJson.module;

writeFile('./rust/pkg/package.json', JSON.stringify(packageJson, null, '\t')).then(() => {
	console.log('Successfully updated package.json!');
});
