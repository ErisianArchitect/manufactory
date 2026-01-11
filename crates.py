import os, sys, shutil
from subprocess import run
from pathlib import Path

from cmdr import (
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

def new(name: str, type: str = 'lib'):
    run(('cargo', 'init' , f'--{type}', f'crates\\{name}'))

def new_command(args):
    return run(('cargo', 'init' , f'--{args.type}', f'crates\\{args.name}')).returncode

def remove(name: str):
    subpath = get_crates_path(name)
    if not subpath.exists():
        raise KeyError(name)
    shutil.rmtree(subpath)

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

def open_terminal(name: str, profile: str | None = None, new_window: bool = False):
    wt_args = ['wt']
    if not new_window:
        wt_args.extend(('-w', '0'))
    subpath = get_crates_path(name)
    wt_args.extend(('-d', str(subpath)))
    if profile:
        wt_args.extend(('-p', profile))
    run(wt_args)

def term_command(args):
    wt_args = ['wt']
    if not args.new_window:
        wt_args.extend(('-w', '0'))
    subpath = get_crates_path(args.name)
    wt_args.extend(('-d', str(subpath)))
    wt_args.extend(('-p', args.profile))
    return run(wt_args).returncode

def exists(name: str)->bool:
    subpath = get_crates_path(name)
    return subpath.exists()

def exists_command(args):
    if exists(args.name):
        if not args.quiet:
            print(f"Exists.")
    else:
        if not args.quiet:
            print(f"Does not exist.")
        return 1

def run_command(args):
    path = get_crates_path(args.name)
    if not path.exists():
        print(f"{args.name} does not exist.")
        return 1
    man_path = path.joinpath('Cargo.toml')
    cargo_args = ('cargo', 'run', '--release', '--manifest-path', f'{man_path}')
    return run(cargo_args).returncode

def debug_command(args):
    path = get_crates_path(args.name)
    if not path.exists():
        print(f"{args.name} does not exist.")
        return 1
    man_path = path.joinpath('Cargo.toml')
    cargo_args = ('cargo', 'run', '--manifest-path', f'{man_path}')
    return run(cargo_args).returncode

_commands = dict(
    new=new_command,
    rm=rm_command,
    term=term_command,
    exists=exists_command,
    run=run_command,
    debug=debug_command,
)

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
        run=subcommand(
            help="Call `cargo run --release` on crate.",
        ).args(
            arg('name', type=str, help=_name_help),
        ),
        debug=subcommand(
            help="Call `cargo run` on crate.",
        ).args(
            arg('name', type=str, help=_name_help),
        )
    )
)
def main(args):
    cmd_func = _commands.get(args.command, None)
    if cmd_func is None:
        print(f'"{args.command}" command not implemented.')
        return 1
    return cmd_func(args)

if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]) or 0)