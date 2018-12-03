use racr::*;



#[test]
fn display_register() {
    let reg = RegisterDefinition {
        access: Access::ReadWrite,
        ident: Ident::from("RegisterName"),
        description: Some(String::from(" description")),

        size: 32,
        reset_value: Some(0),

        fields: vec![
            FieldInstance{ident: Ident::from("field0"), description: None, bit_start: 0, bit_end: 7, access: Some(Access::ReadWrite)},
            FieldInstance{ident: Ident::from("field1"), description: None, bit_start: 8, bit_end: 15, access: Some(Access::ReadOnly)},
            FieldInstance{ident: Ident::from("field2"), description: None, bit_start: 16, bit_end: 23, access: Some(Access::WriteOnly)},
            FieldInstance{ident: Ident::from("field3"), description: None, bit_start: 24, bit_end: 31, access: None},
        ],
    };

    let mut display_string = format!("{}", reg);

    let mut desired_string = String::from(
        "/// description
ReadWrite register[32] RegisterName = 0x0 {
    ReadWrite field0[0..7],
    ReadOnly field1[8..15],
    WriteOnly field2[16..23],
    field3[24..31],
}"
    );


    // Strip whitespaces and compare
    display_string.retain(|c| c!= '\n' && c != ' ');
    desired_string.retain(|c| c!= '\n' && c != ' ');

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_peripheral() {
    let reg = PeripheralDefinition {
        ident: Ident::from("PeripheralName"),
        description: Some(String::from(" description")),

        registers: vec![
            RegisterSlot::Single{instance: RegisterInstance{ident: Ident::from("reg0"), ty: RegisterType::Single{path: Ident::from("RegisterName").into()}}, offset: 0x0},
            RegisterSlot::Single{instance: RegisterInstance{ident: Ident::from("reg1"), ty: RegisterType::Single{path: Ident::from("RegisterName").into()}}, offset: 0x4},
            RegisterSlot::Single{instance: RegisterInstance{ident: Ident::from("reg2"), ty: RegisterType::Single{path: Ident::from("RegisterName").into()}}, offset: 0x8},
            RegisterSlot::Single{instance: RegisterInstance{ident: Ident::from("reg3"), ty: RegisterType::Single{path: Ident::from("RegisterName").into()}}, offset: 0xc},
            RegisterSlot::Overloaded{alternatives: vec![
                    RegisterInstance{ident: Ident::from("reg4a"), ty: RegisterType::Single{path: Ident::from("A").into()}},
                    RegisterInstance{ident: Ident::from("reg4b"), ty: RegisterType::Single{path: Ident::from("B").into()}},
                    RegisterInstance{ident: Ident::from("reg4c"), ty: RegisterType::Single{path: Ident::from("C").into()}},
                ], offset: 0x10
            },
            RegisterSlot::Single{instance: RegisterInstance{ident: Ident::from("reg5"), ty: RegisterType::Array{path: Ident::from("RegisterName").into(), size: 3}}, offset: 0x14},
        ],
    };

    let mut display_string = format!("{}", reg);

    let mut desired_string = String::from(
        "/// description
peripheral PeripheralName {
    reg0: RegisterName @ 0x0,
    reg1: RegisterName @ 0x4,
    reg2: RegisterName @ 0x8,
    reg3: RegisterName @ 0xc,
    (reg4a: A | reg4b: B | reg4c: C) @ 0x10,
    reg5: [RegisterName; 3]@ 0x14,
}"
    );


    // Strip whitespaces and compare
    display_string.retain(|c| c!= '\n' && c != ' ');
    desired_string.retain(|c| c!= '\n' && c != ' ');

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_device() {
    let device = DeviceDefinition {
        ident: Ident::from("UnitName"),
        description: Some(String::from(" description")),

        peripherals: vec![
            PeripheralInstance{ident: Ident::from("peripheral0"), path: Ident::from("PeripheralName").into(), address: 0x4000_0000,},
            PeripheralInstance{ident: Ident::from("peripheral1"), path: Ident::from("PeripheralName").into(), address: 0x4000_2000},
            PeripheralInstance{ident: Ident::from("peripheral2"), path: Ident::from("PeripheralName").into(), address: 0x4000_4000},
            PeripheralInstance{ident: Ident::from("peripheral3"), path: Ident::from("PeripheralName").into(), address: 0x4000_8000},
        ],
    };

    let mut display_string = format!("{}", device);

    let mut desired_string = String::from(
        "/// description
peripheral UnitName {
    peripheral0: PeripheralName @ 0x40000000,
    peripheral1: PeripheralName @ 0x40002000,
    peripheral2: PeripheralName @ 0x40004000,
    peripheral3: PeripheralName @ 0x40008000,
}"
    );


    // Strip whitespaces and compare
    display_string.retain(|c| c!= '\n' && c != ' ');
    desired_string.retain(|c| c!= '\n' && c != ' ');

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_used() {

    let used = Use {
        tree: UseTree::Path {
            path_segment: "foo".into(),
            sub_tree: Box::new(
                UseTree::Path {
                    path_segment: "bar".into(),
                    sub_tree: Box::new( UseTree::Ident("Baz".into()) )
                }
            )
        }
    };
        
    let mut display_string = format!("{}", used);

    let mut desired_string = String::from("use foo::bar::Baz;");

    // Strip whitespaces and compare
    display_string.retain(|c| c!= '\n' && c != ' ');
    desired_string.retain(|c| c!= '\n' && c != ' ');

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_mod_without_content() {
    let module = Module {
        ident: "foo".into(),
        content: None,
    };
        
    let mut display_string = format!("{}", module);

    let mut desired_string = String::from("mod foo;");

    // Strip whitespaces and compare
    display_string.retain(|c| c!= '\n' && c != ' ');
    desired_string.retain(|c| c!= '\n' && c != ' ');

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_mod_with_content() {
    let module = Module {
        ident: "foo".into(),
        content: Some(vec![
            Item::Use(Use{tree: UseTree::Ident("bar".into())}),
            Item::Use(Use{tree: UseTree::Ident("baz".into())}),
        ]),
    };
        
    let mut display_string = format!("{}", module);

    let mut desired_string = String::from("mod foo { use bar; use baz; }");

    // Strip whitespaces and compare
    display_string.retain(|c| c!= '\n' && c != ' ');
    desired_string.retain(|c| c!= '\n' && c != ' ');

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_file() {
    let file_content = FileContent {
        content: vec![
            Item::Use(Use{tree: UseTree::Ident("foo".into())}),
            Item::Use(Use{tree: UseTree::Ident("bar".into())}),
            Item::Use(Use{tree: UseTree::Ident("baz".into())}),
        ],
    };
        
    let mut display_string = format!("{}", file_content);

    let mut desired_string = String::from("use foo; use bar; use baz;");

    // Strip whitespaces and compare
    display_string.retain(|c| c!= '\n' && c != ' ');
    desired_string.retain(|c| c!= '\n' && c != ' ');

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_path() {
    let path = Path {
        segments: vec![
            Ident::from("foo"),
            Ident::from("bar"),
            Ident::from("baz"),
        ],
    };

    let display_string = format!("{}", path);

    let desired_string = String::from("foo::bar::baz");

    // Compare
    assert_eq!(
        display_string,
        desired_string
    )
}
