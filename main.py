import argparse
from pathlib import Path

import pandas as pd


def main(i_file: Path) -> None:
    data = pd.read_csv(i_file, sep=',')
    pass


if __name__ == '__main__':

    parser = argparse.ArgumentParser(description='Garmin Connect importer')
    parser.add_argument('-i', '--input', type=Path, required=True, help='csv file exported from garmin connect')

    args = parser.parse_args()

    main(args.input)
