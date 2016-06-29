set nocompatible
set runtimepath+=~/.vim/dein/repos/github.com/Shougo/dein.vim " path to dein.vim


" Run using command => :call dein#install()
call dein#begin(expand('~/.vim/dein')) " plugins' root path

call dein#add('Shougo/dein.vim')

" lazy load on command executed
call dein#add('scrooloose/nerdtree',
            \{'on_cmd': 'NERDTreeToggle'})
call dein#add('Xuyuanp/nerdtree-git-plugin')
call dein#add('jistr/vim-nerdtree-tabs')

call dein#add('Shougo/unite.vim')

call dein#add('honza/vim-snippets')

call dein#add('nanotech/jellybeans.vim')

call dein#add('tpope/vim-fugitive')

call dein#add('vim-airline/vim-airline')
call dein#add('vim-airline/vim-airline-themes')

call dein#add('scrooloose/syntastic')

" and a lot more plugins.....

call dein#end()
filetype plugin indent on

" show existing tab with 4 spaces width
set tabstop=4

" when indenting with '>', use 4 spaces width
set shiftwidth=4

" On pressing tab, insert 4 spaces
set expandtab

" for Ctrl-P fuzzy search
set runtimepath^=~/.vim/bundle/ctrlp.vim

" for jellybean colour theme
set t_Co=256
colorscheme jellybeans
set background=dark

" for NERDTree - open tree with current file directory
autocmd BufEnter * lcd %:p:h 

