/// <reference types="vite/client" />

interface ImportMetaEnv {
	readonly GH_PAGES_BASE_URL: string;
}
interface ImportMeta {
	readonly env: ImportMetaEnv;
}
