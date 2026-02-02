# Violet Project Generator (VPG) 設定ファイル仕様書 v0.6

## 概要

VPGの設定ファイル（sample.toml）は、Violetハイパーバイザの仮想マシン設定、物理H/W設定、カスタムカーネル設定を記載するためのTOML形式の設定ファイルです。

## 基本構造

```toml
# 基本情報
version = 0.6
name = "Example Project"
test = false

[vm_*]
# プロジェクト・VM設定

[kernel]
# カーネル設定

[env]
# 環境・物理H/W設定
```

---

## 詳細仕様

### 1. 基本設定

```toml
version = 0.6
name = "Example Project"
test = false  # Optional: デフォルトは false
```

- **version**: 対応するVioletのバージョン（float型）
- **name**: プロジェクト名（string型）
- **[test]**: テストモードの有効/無効（bool型:false）

### 2. VM設定 `[vm_*]`

```toml
[vm_linux]
os = "linux"
os_version = "6.1.0"

[vm_linux.cpus]
num_cpus = 2
pcpu_mapping = [1, 2]  # vcpu0->pcpu1, vcpu1->pcpu2

[[vm_linux.memory]]
guest_base = 0x80000000
host_base = 0x80000000
size = 0x8000000

[[vm_linux.memory]]
guest_base = 0x88100000
host_base = 0x88100000
size = 0x200000

[[vm_linux.vdev]]
device = "vplic"
guest_base = 0x0c000000
size = 0x400000

[vm_linux.vdev.config]
vcpu_mapping = [1, 0]

[vm_freertos]
os = "freertos"
os_version = "10.4.6"

[vm_freertos.cpus]
num_cpus = 1
pcpu_mapping = [3]  # vcpu0->pcpu3

[[vm_freertos.memory]]
guest_base = 0x80000000
host_base = 0x90000000
size = 0x1000000

[[vm_freertos.vdev]]
device = "vuart"
guest_base = 0x10000000
size = 0x20
```

#### VM設定 `[vm_任意の名前]`
各VMは`[vm_任意の名前]`形式で定義します。例：`[vm_linux]`, `[vm_freertos]`など

- **os**: ゲストOS種別（"linux", "freertos", "bare-metal"など）
- **os_version**: ゲストOSのバージョン（string型）- *オプション*

##### CPU設定 `[vm_名前.cpus]`
- **num_cpus**: 仮想CPU数（int型）
- **pcpu_mapping**: 物理CPUマッピング配列（int array型）
  - 配列のインデックスが仮想CPU ID、値が物理CPU ID
  - 例：`[1, 2]` = vcpu0→pcpu1, vcpu1→pcpu2

##### メモリ設定 `[[vm_名前.memory]]`（配列形式）
- **guest_base**: ゲスト物理アドレス（hex int型）
- **host_base**: ホスト物理アドレス（hex int型）
- **size**: メモリ領域サイズ（hex int型）

##### 仮想デバイス設定 `[[vm_名前.vdev]]`（配列形式）
- **device**: 仮想デバイス種別（"vplic", "vclint", "vuart", "vgpio"など、string型）
- **guest_base**: ゲスト側でのベースアドレス（hex int型）
- **size**: デバイスのアドレス空間サイズ（hex int型）
- **config**: デバイス固有の設定（object型）- *オプション*

### 3. カーネル設定 `[kernel]`

```toml
[kernel]
scheduler = "fifo"
dispatcher = "minimal"
heap_size = 0x20000
stack_size = 0x4000
```

- **scheduler**: スケジューラ種別（"fifo", "rr", "cfs"など）
- **dispatcher**: ディスパッチャ種別（"minimal", "full"など）
- **heap_size**: ヒープサイズ（hex int型）
- **stack_size**: スタックサイズ（hex int型）

### 4. 環境・物理H/W設定 `[env]`

```toml
[env]
arch = "rv64"
hext = true
num_of_cpus = 2
cores = [0, 1]
bsp = 0

[env.memory]
base = 0x80000000
size = 0x40000000

[[env.device]]
device = "uart"
base = 0x10000000
size = 0x20

[[env.device]]
device = "clint"
base = 0x02000000
size = 0x10000

[[env.device]]
device = "plic"
base = 0x0c000000
size = 0x400000
```

#### 基本環境設定
- **arch**: アーキテクチャ（"rv64", "rv32", "x86_64"など）
- **hext**: ハイパーバイザ拡張の有効/無効（bool型）
- **num_of_cpus**: 物理CPU数（int型）
- **cores**: 使用するCPUコア番号の配列（int array型）
- **bsp**: ブートストラッププロセッサのコア番号（int型）

#### 物理メモリ設定 `[env.memory]`
- **base**: 物理メモリのベースアドレス（hex int型）
- **size**: 物理メモリサイズ（hex int型）

#### デバイス設定 `[[env.device]]`（配列形式）
- **device**: デバイス種別（"uart", "clint", "plic", "gpio", "spi", "i2c"など、string型）
- **base**: デバイスのベースアドレス（hex int型）
- **size**: デバイスのアドレス空間サイズ（hex int型）

#### QEMU設定 `[env.qemu]`（オプション）
- **kernel_path**: カーネルイメージのパス（string型、オプション）
- **initrd_path**: initrdイメージのパス（string型、オプション）
- **dtb_path**: Device Tree Blobのパス（string型、オプション）
- **bios_path**: BIOSイメージのパス（string型、オプション）

---

## 使用例

```toml
version = 0.6
name = "Multi-OS Project"
test = false  # オプション項目

[vm_linux]
os = "linux"
os_version = "6.1.0"

[vm_linux.cpus]
num_cpus = 2
pcpu_mapping = [1, 2]

[[vm_linux.memory]]
guest_base = 0x80000000
host_base = 0x80000000
size = 0x8000000

[[vm_linux.vdev]]
device = "vplic"
guest_base = 0x0c000000
size = 0x400000

[vm_linux.vdev.config]
vcpu_mapping = [1, 0]

[vm_freertos]
os = "freertos"
os_version = "10.4.6"

[vm_freertos.cpus]
num_cpus = 1
pcpu_mapping = [3]

[[vm_freertos.memory]]
guest_base = 0x80000000
host_base = 0x90000000
size = 0x1000000

[kernel]
scheduler = "fifo"
heap_size = 0x20000
stack_size = 0x4000

[env]
arch = "rv64"
hext = true
num_of_cpus = 4
cores = [0, 1, 2, 3]
bsp = 0

[env.memory]
base = 0x80000000
size = 0x40000000

[env.qemu]
kernel_path = "/opt/riscv/linux/vmlinux.bin"
initrd_path = "/opt/riscv/busybox/_install"

[[env.device]]
device = "uart"
base = 0x10000000
size = 0x20

[[env.device]]
device = "clint"
base = 0x02000000
size = 0x10000

[[env.device]]
device = "plic"
base = 0x0c000000
size = 0x400000
```

---
