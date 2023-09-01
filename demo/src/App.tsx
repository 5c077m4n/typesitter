import React, { useEffect, useMemo, useState } from 'react';
import ReactJson from 'react-json-view';
import init, { tokenize, build_ast as buildAST, ts_eval as tsEval, check } from 'js_bind';

function Main() {
	const [script, setScript] = useState('');

	const { tokens, time: tokenTime } = useMemo(() => {
		try {
			const start = performance.now();
			const t = tokenize(script);
			return { tokens: JSON.parse(t), time: performance.now() - start };
		} catch (e) {
			return {};
		}
	}, [script]);
	const { errors, time: errorTime } = useMemo(() => {
		try {
			const start = performance.now();
			const t = check(script);
			return { errors: JSON.parse(t), time: performance.now() - start };
		} catch (e) {
			return {};
		}
	}, [script]);
	const { ast, time: astTime } = useMemo(() => {
		try {
			const start = performance.now();
			const tree = buildAST(script);
			return { ast: JSON.parse(tree), time: performance.now() - start };
		} catch (e) {
			return {};
		}
	}, [script]);
	const { result, time: evalTime } = useMemo(() => {
		try {
			const start = performance.now();
			const result = tsEval(script);
			return { result, time: performance.now() - start };
		} catch (e) {
			return {};
		}
	}, [script]);

	return (
		<main className="w-full h-full p-4 space-y-4">
			<textarea
				className="p-2 w-full border border-black rounded font-mono"
				value={script}
				onChange={(e) => setScript(e.target.value)}
			/>
			<details>
				<summary>Tokenizer ({tokenTime}ms)</summary>
				<ReactJson src={tokens} />
			</details>
			<details>
				<summary>Errors ({errorTime}ms)</summary>
				<ReactJson src={errors} />
			</details>
			<details>
				<summary>AST ({astTime}ms)</summary>
				<ReactJson src={ast} />
			</details>
			<details open>
				<summary>Result ({evalTime}ms)</summary>
				<textarea className="p-2 w-full border border-black rounded font-mono" value={result} disabled />
			</details>
		</main>
	);
}

export function App() {
	const [isReady, setIsReady] = useState(false);
	useEffect(() => {
		init().then(() => setIsReady(true));
	}, []);

	return isReady ? <Main /> : <p>Loading...</p>;
}
