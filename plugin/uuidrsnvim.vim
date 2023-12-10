let s:p_dir = expand('<sfile>:p:h')
let g:is_running = 0
let g:channel = -1

function! StartIfNotRunning()
    if g:is_running == 0
        exec ':cd ' . s:p_dir
        let g:channel = rpcstart('../target/release/uuid-nvim', [])
    endif
endfunction

function! uuidrsnvim#build()
    exec ':cd ' . s:p_dir . '/..'
    make
endfunction

function! uuidrsnvim#uuid()
    call StartIfNotRunning()
    let uuid = rpcrequest(g:channel, 'uuid', [])
    echo uuid
    let @" = uuid
endfunction

command! Uuid call uuidrsnvim#uuid()
command! UuidBuild call uuidrsnvim#build()

