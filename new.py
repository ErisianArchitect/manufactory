import builtins
from subprocess import run
from cmdr import entry, command, flag, arg, group, subcommand, subparser

@entry
@command(
    "new",
    description="Create new crates within this workspace."
).args(
    arg('type', type=str, choices=['bin', 'lib']),
    arg('name', type=str),
)
def main(args):
    return run(('cargo', 'init', f'--{args.type}', f'crates\\{args.name}'))