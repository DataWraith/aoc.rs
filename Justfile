test:
     cargo nextest run

generate day:
     cargo generate --vcs none -p template --name day-{{day}}

