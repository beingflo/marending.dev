#!/usr/bin/env bash
die() { echo "$*" 1>&2 ; exit 1; }

echo -e "Deploying marending.dev to production!"

[ -z "$(git status --porcelain)" ] || die "There are uncommitted changes"

cd service; version=$(cargo metadata --format-version=1 --no-deps | jq '.packages[0].version' | tr -d '"'); cd ..

echo "Latest version: ${version}"

next_version=$(echo ${version} | awk -F. -v OFS=. '{$NF += 1 ; print}')

read -p "Enter version to be deployed [${next_version}]: " new_version 

new_version=${new_version:-${next_version}}

cd service; cargo set-version "${new_version}" || die "Failed to set version in Cargo.toml"; cd ..

docker buildx build -t "ghcr.io/beingflo/marending-dev:${new_version}" . || die "Failed to build docker image"
docker push "ghcr.io/beingflo/marending-dev:${new_version}" || die "Failed to push docker image"

sed -i '' -e "s/image: \"ghcr.io\/beingflo\/marending-dev:${version}\"/image: \"ghcr.io\/beingflo\/marending-dev:${new_version}\"/" ./compose.prod.yml || die "Failed to write new version to docker compose file"

docker --context arm compose --file compose.prod.yml pull || die "Failed to pull new image"
docker --context arm compose --file compose.prod.yml up -d || die "Failed to bring compose up"

git commit -am "Release ${new_version}"
git tag "${new_version}"
git push origin --tags