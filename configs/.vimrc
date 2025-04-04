" do not check for compatability with vi
set nocompatible

" colors
set term=xterm-256color
set termguicolors


" show special characters
" set list " show $ symbol in the end of the line
set linebreak
set colorcolumn=120
set textwidth=120

" syntax highlight and indent
filetype indent on
syntax on
set tabstop=4
set shiftwidth=4
set softtabstop=4
set wrap
set expandtab
set smarttab
set autoindent
set cursorline
set showmatch
set modeline
set number
set relativenumber
set mouse=a
set backspace=start,eol,indent

" search for something in code
set hlsearch
set ignorecase
set smartcase
set laststatus=2

" color scheme
colorscheme wildcharm
"highlight Cursorline cterm=bold ctermbg=black
"highlight Normal guibg=NONE ctermbg=NONE
