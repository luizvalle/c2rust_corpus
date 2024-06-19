use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_node {
    pub left: *mut tree_node,
    pub right: *mut tree_node,
    pub key: libc::c_int,
    pub data: *mut libc::c_void,
}
pub type splay_tree = tree_node;
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn splaytree_splay_nonnull(
    mut t: *mut splay_tree,
    mut i: libc::c_int,
) -> *mut splay_tree {
    let mut N: splay_tree = splay_tree {
        left: 0 as *mut tree_node,
        right: 0 as *mut tree_node,
        key: 0,
        data: 0 as *mut libc::c_void,
    };
    let mut l: *mut splay_tree = 0 as *mut splay_tree;
    let mut r: *mut splay_tree = 0 as *mut splay_tree;
    let mut y: *mut splay_tree = 0 as *mut splay_tree;
    N.right = 0 as *mut tree_node;
    N.left = N.right;
    r = &mut N;
    l = r;
    loop {
        if i < (*t).key {
            if ((*t).left).is_null() {
                break;
            }
            if i < (*(*t).left).key {
                y = (*t).left;
                (*t).left = (*y).right;
                (*y).right = t;
                t = y;
                if ((*t).left).is_null() {
                    break;
                }
            }
            (*r).left = t;
            r = t;
            t = (*t).left;
        } else {
            if !(i > (*t).key) {
                break;
            }
            if ((*t).right).is_null() {
                break;
            }
            if i > (*(*t).right).key {
                y = (*t).right;
                (*t).right = (*y).left;
                (*y).left = t;
                t = y;
                if ((*t).right).is_null() {
                    break;
                }
            }
            (*l).right = t;
            l = t;
            t = (*t).right;
        }
    }
    (*l).right = (*t).left;
    (*r).left = (*t).right;
    (*t).left = N.right;
    (*t).right = N.left;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn splaytree_insert_splayed(
    mut t: *mut splay_tree,
    mut i: libc::c_int,
    mut data: *mut libc::c_void,
) -> *mut splay_tree {
    let new: *mut splay_tree = malloc(
        ::core::mem::size_of::<splay_tree>() as libc::c_ulong,
    ) as *mut splay_tree;
    if !new.is_null() {} else {
        __assert_fail(
            b"new\0" as *const u8 as *const libc::c_char,
            b"algo_splaytree.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"splay_tree *splaytree_insert_splayed(splay_tree *, int, void *)\0"))
                .as_ptr(),
        );
    }
    'c_1135: {
        if !new.is_null() {} else {
            __assert_fail(
                b"new\0" as *const u8 as *const libc::c_char,
                b"algo_splaytree.c\0" as *const u8 as *const libc::c_char,
                114 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"splay_tree *splaytree_insert_splayed(splay_tree *, int, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if t.is_null() {
        (*new).right = 0 as *mut tree_node;
        (*new).left = (*new).right;
    } else if i < (*t).key {
        (*new).left = (*t).left;
        (*new).right = t;
        (*t).left = 0 as *mut tree_node;
    } else {
        (*new).right = (*t).right;
        (*new).left = t;
        (*t).right = 0 as *mut tree_node;
    }
    (*new).key = i;
    (*new).data = data;
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn splaytree_insert(
    mut t: *mut splay_tree,
    mut i: libc::c_int,
    mut data: *mut libc::c_void,
) -> *mut splay_tree {
    return if !t.is_null()
        && {
            t = splaytree_splay_nonnull(t, i);
            (*t).key == i
        }
    {
        t
    } else {
        splaytree_insert_splayed(t, i, data)
    };
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn splaytree_delete_splayed_node(
    mut t: *mut splay_tree,
) -> *mut splay_tree {
    let mut x: *mut splay_tree = (*t).right;
    if !((*t).left).is_null() {
        x = splaytree_splay_nonnull((*t).left, (*t).key);
        (*x).right = (*t).right;
    }
    free(t as *mut libc::c_void);
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn splaytree_delete(
    mut t: *mut splay_tree,
    mut i: libc::c_int,
) -> *mut splay_tree {
    return if !t.is_null()
        && {
            t = splaytree_splay_nonnull(t, i);
            (*t).key == i
        }
    {
        splaytree_delete_splayed_node(t)
    } else {
        t
    };
}
