#!/bin/bash

export ENV_UNIQUE_ELEMENT_LIMIT=16777216

export ENV_OUTPUT_MODE=plain
export ENV_OUTPUT_MODE=json

sampletxt=./sample.d/input.txt

geninput(){
	echo generating input...
	seq 1 16777216 |
		cat <(printf '%s\n' 1 1 1 1 1 1 1 1 1) /dev/stdin |
		cat > "${sampletxt}"
}

run_native(){
	echo
	echo --------------------------------
	echo native
	cat "${sampletxt}" |
		\time -l ./rs-sort-uniq-count |
		sort -n |
		tail -3
}

run_wazero(){
	echo
	echo --------------------------------
	echo wazero
	cat "${sampletxt}" |
		\time \
			-l \
			wazero \
				run \
				--env ENV_UNIQUE_ELEMENT_LIMIT=$ENV_UNIQUE_ELEMENT_LIMIT \
				./rs-sort-uniq-count.wasm |
		sort -n |
		tail -3
}

run_wasmer(){
	echo
	echo --------------------------------
	echo wazero
	cat "${sampletxt}" |
		\time \
			-l \
			wasmer \
				run \
				--env ENV_UNIQUE_ELEMENT_LIMIT=$ENV_UNIQUE_ELEMENT_LIMIT \
				./rs-sort-uniq-count.wasm |
		sort -n |
		tail -3
}

run_wasmtime(){
	echo
	echo --------------------------------
	echo wazero
	cat "${sampletxt}" |
		\time \
			-l \
			wasmtime \
				run \
				--env ENV_UNIQUE_ELEMENT_LIMIT=$ENV_UNIQUE_ELEMENT_LIMIT \
				./rs-sort-uniq-count.wasm |
		sort -n |
		tail -3
}

run_unix(){
	echo
	echo --------------------------------
	echo unix
	cat "${sampletxt}" |
		\time -l sort |
		uniq -c |
		sort -n |
		tail -3
}

test -f "${sampletxt}" || geninput

run_native 2>&1 | fgrep -e resident -e real -e 10
run_wasmer 2>&1 | fgrep -e resident -e real -e 10
run_wasmtime 2>&1 | fgrep -e resident -e real -e 10
run_wazero 2>&1 | fgrep -e resident -e real -e 10
run_unix   2>&1 | fgrep -e resident -e real -e 10
