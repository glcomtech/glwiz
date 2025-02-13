set mouse=a
syntax on
set number
set cursorline
:highlight Cursorline cterm=bold ctermbg=black
set hlsearch
set ignorecase
set smartcase
set tabstop=4
set softtabstop=4
set shiftwidth=4
set expandtab
set autoindent
set showmatch
set scrolloff=5
set laststatus=2
set textwidth=79
set wrap

if !has('gui_running')
	set t_Co=256
endif

set termguicolors
colorscheme wildcharm
set background=dark
highlight Normal guibg=NONE ctermbg=NONE
