let s:c_dir = system('pwd')
let s:p_dir = expand('<sfile>:p:h')
let s:bin = s:p_dir . '/../target/release/uuid-nvim'
let g:is_running = 0
let g:channel = -1

function! StartIfNotRunning()
    if g:is_running == 0
        let g:channel = rpcstart(s:bin, [])
    endif
endfunction

function! uuidrsnvim#build()
    exec ':cd ' . s:p_dir . '/..'
    make
    exec ':cd ' . s:c_dir
endfunction

function! uuidrsnvim#uuid()
    call StartIfNotRunning()
    let uuid = rpcrequest(g:channel, 'uuid', [])
    echo uuid
    let @" = uuid
endfunction

command! Uuid call uuidrsnvim#uuid()
command! UuidBuild call uuidrsnvim#build()

