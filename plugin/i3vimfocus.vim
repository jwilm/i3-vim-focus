func! Focus(comando,vim_comando)
  let oldw = winnr()
  silent exe 'wincmd ' . a:vim_comando
  let neww = winnr()
  if oldw == neww
      call i3vimfocus#PythonExecProcess("i3-msg", ["-q", "focus", a:comando])
  endif
endfunction
