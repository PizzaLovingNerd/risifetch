Name:           risifetch
Version:        1.0
Release:        1%{?dist}
Summary:        risiOS's fork of treefetch for a lightweight neofetch alternative 

License:        GPL v3
URL:            https://github.com/risiOS/risifetch
Source0:        https://github.com/risiOS/risifetch/archive/refs/heads/main.tar.gz

BuildArch:	noarch
BuildRequires:	rust
BuildRequires:	cargo

%description
risiOS's fork of treefetch for a lightweight neofetch alternative

%prep
%autosetup -n %{name}-main

%build
cargo build

%install
mkdir -p %{buildroot}%{_bindir}
cp target/debug/risifetch %{buildroot}%{_bindir}

%files
%{_bindir}/risifetch

%changelog
* Tue Apr 27 2022 PizzaLovingNerd
- First spec file
