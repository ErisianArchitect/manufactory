import os, sys, shutil
from subprocess import run
from pathlib import Path

from cmdr import (
    entry,
    command,
    subparser,
    subcommand,
    flag,
    arg,
    group,
)

# This script should exist in the root of the workspace crate that
# has the `crates` directory.
_root_dir = Path(__file__).parent
_crates_dir = _root_dir.joinpath('crates')

def get_crates_path(subpath: os.PathLike)->Path:
    return _crates_dir.joinpath(subpath)

def confirm(prompt: str)->bool:
    user_input = input(f'{prompt} [Yes/No]:').lower()
    match user_input:
        case 'yes' | 'y':
            return True
        case _:
            return False

def new_command(args):
    return run(('cargo', 'init' , f'--{args.type}', f'crates\\{args.name}')).returncode

def rm_command(args):
    subpath = get_crates_path(args.name)
    if not subpath.exists():
        if not args.quiet:
            print('Path does not exist.')
        return 1
    if args.force or confirm(f'Permanently delete {args.name}?'):
        shutil.rmtree(subpath)
        if not args.quiet:
            print(f'{args.name} was deleted.')

def term_command(args):
    wt_args = ['wt']
    if not args.new_window:
        wt_args.extend(('-w', '0'))
    subpath = get_crates_path(args.name)
    wt_args.extend(('-d', str(subpath)))
    wt_args.extend(('-p', args.profile))
    return run(wt_args).returncode

def exists_command(args):
    subpath = get_crates_path(args.name)
    if subpath.exists():
        if not args.quiet:
            print(f"Exists.")
    else:
        if not args.quiet:
            print(f"Does not exist.")
        return 1

_name_help = 'The name of the subcrate.'

@command(
    "crates",
    description="Manage crates inside of Rust project (\"./crates/*\")",
).args(
    flag('--quiet', '-q', help='Suppress printing.'),
).subcommand(
    subparser(
        title="Command",
        dest="command",
        required=True,
    ).commands(
        new=subcommand(
            help="Create a new crate within the manufactory workspace.",
        ).args(
            arg('type', choices=['bin', 'lib'], help='The type of crate to create.'),
            arg('name', help=_name_help)
        ),
        rm=subcommand(
            help="Remove a crate from within the manufactory workspace.",
        ).args(
            flag('--force', '-f', help='Force removal of crate.'),
            arg('name', help=_name_help),
        ),
        term=subcommand(
            help="Open up Windows Terminal session in the crate's root.",
        ).args(
            flag('--new-window', '-n', dest='new_window', help='Open terminal session in new window instead of reusing an existing one.'),
            arg('--profile', '-p', type=str, default='cmd', required=False),
            arg('name', type=str, help=_name_help),
        ),
        exists=subcommand(
            help="Check if a crate exists in the manufactory workspace.",
        ).args(
            arg('name', type=str, help=_name_help),
        ),
    )
)
def main(args):
    match args.command:
        case "new":
            return new_command(args)
        case "rm":
            return rm_command(args)
        case "term":
            return term_command(args)
        case "exists":
            return exists_command(args)
        case other:
            print(f'"{other}" command not implemented.')

if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]) or 0)