#!/usr/bin/env bash

BOM_CMD='cargo run --bin bom-client -- '

# Create component parts
function add_part()
{
    result=$($BOM_CMD create-part --name "$1")
    id=$(echo $result | sed 's/.*"id": "\([0-9a-z-]*\)".*/\1/')
    echo $id
}

METAL_BARREL_TOP_ID=$(add_part "metal barrel top")
METAL_BARREL_BOTTOM_ID=$(add_part "metal barrel bottom")
PLASTIC_BARREL_TOP_ID=$(add_part "plastic barrel top")
PLASTIC_BARREL_BOTTOM_ID=$(add_part "plastic barrel bottom")
POCKET_CLIP_ID=$(add_part "pocket clip")
THRUSTER_ID=$(add_part "thruster")
CAM_ID=$(add_part "cam")
RUBBER_GRIP_ID=$(add_part "rubber grip")
SPRING_ID=$(add_part "spring")
CART_BODY_ID=$(add_part "cartridge body")
CART_CAP_ID=$(add_part "cartridge cap")
WRITING_TIP_ID=$(add_part "writing tip")
RED_INK_ID=$(add_part "red ink")
BLUE_INK_ID=$(add_part "blue ink")
BLACK_INK_ID=$(add_part "black ink")
BOX_TOP_ID=$(add_part "box top")
BOX_BOTTOM_ID=$(add_part "box bottom")
BOX_INSERT_ID=$(add_part "box insert")

# Compose subassemblies
function update_part()
{
    id=$1; shift;
    result=$($BOM_CMD update-part --id $id --children $@)
    echo $result
}
# Top assembly variants
METAL_TOP_ASSY_ID=$(add_part "metal top assembly")
update_part $METAL_TOP_ASSY_ID $METAL_BARREL_TOP_ID $POCKET_CLIP_ID $THRUSTER_ID $CAM_ID

PLASTIC_TOP_ASSY_ID=$(add_part "plastic top assembly")
update_part $PLASTIC_TOP_ASSY_ID $PLASTIC_BARREL_TOP_ID $POCKET_CLIP_ID $THRUSTER_ID $CAM_ID

# Bottom assembly variants
METAL_BOTTOM_ASSY_ID=$(add_part "metal bottom assembly")
update_part $METAL_BOTTOM_ASSY_ID $METAL_BARREL_BOTTOM_ID $RUBBER_GRIP_ID

PLASTIC_BOTTOM_ASSY_ID=$(add_part "plastic bottom assembly")
update_part $PLASTIC_BOTTOM_ASSY_ID $PLASTIC_BARREL_BOTTOM_ID $RUBBER_GRIP_ID

# Ink cartridge variants
RED_INK_CART_ID=$(add_part "red ink cartridge")
update_part $RED_INK_CART_ID $RED_INK_ID $CART_BODY_ID $CART_CAP_ID $WRITING_TIP_ID

BLUE_INK_CART_ID=$(add_part "blue ink cartridge")
update_part $BLUE_INK_CART_ID $BLUE_INK_ID $CART_BODY_ID $CART_CAP_ID $WRITING_TIP_ID

BLACK_INK_CART_ID=$(add_part "black ink cartridge")
update_part $BLACK_INK_CART_ID $BLACK_INK_ID $CART_BODY_ID $CART_CAP_ID $WRITING_TIP_ID

# Make pen variants
METAL_RED_PEN_ID=$(add_part "metal red pen")
update_part $METAL_RED_PEN_ID $RED_INK_CART_ID $METAL_TOP_ASSY_ID $METAL_BOTTOM_ASSY_ID $SPRING_ID

METAL_BLUE_PEN_ID=$(add_part "metal blue pen")
update_part $METAL_BLUE_PEN_ID $BLUE_INK_CART_ID $METAL_TOP_ASSY_ID $METAL_BOTTOM_ASSY_ID $SPRING_ID

METAL_BLACK_PEN_ID=$(add_part "metal black pen")
update_part $METAL_BLACK_PEN_ID $BLACK_INK_CART_ID $METAL_TOP_ASSY_ID $METAL_BOTTOM_ASSY_ID $SPRING_ID

PLASTIC_RED_PEN_ID=$(add_part "plastic red pen")
update_part $PLASTIC_RED_PEN_ID $RED_INK_CART_ID $PLASTIC_TOP_ASSY_ID $PLASTIC_BOTTOM_ASSY_ID $SPRING_ID

PLASTIC_BLUE_PEN_ID=$(add_part "plastic blue pen")
update_part $PLASTIC_BLUE_PEN_ID $BLUE_INK_CART_ID $PLASTIC_TOP_ASSY_ID $PLASTIC_BOTTOM_ASSY_ID $SPRING_ID

PLASTIC_BLACK_PEN_ID=$(add_part "plastic black pen")
update_part $PLASTIC_BLACK_PEN_ID $BLACK_INK_CART_ID $PLASTIC_TOP_ASSY_ID $PLASTIC_BOTTOM_ASSY_ID $SPRING_ID

# Populate final packages
METAL_RED_PEN_PACKAGE_ID=$(add_part "metal red pen package")
update_part $METAL_RED_PEN_PACKAGE_ID $METAL_RED_PEN_ID $BOX_TOP_ID $BOX_BOTTOM_ID $BOX_INSERT_ID

METAL_BLUE_PEN_PACKAGE_ID=$(add_part "metal blue pen package")
update_part $METAL_BLUE_PEN_PACKAGE_ID $METAL_BLUE_PEN_ID $BBOX_TOP_ID $BOX_BOTTOM_ID $BOX_INSERT_ID

METAL_BLACK_PEN_PACKAGE_ID=$(add_part "metal black pen package")
update_part $METAL_BLACK_PEN_PACKAGE_ID $METAL_BLACK_PEN_ID $BLBOX_TOP_ID $BOX_BOTTOM_ID $BOX_INSERT_ID

PLASTIC_RED_PEN_PACKAGE_ID=$(add_part "plastic red pen package")
update_part $PLASTIC_RED_PEN_PACKAGE_ID $PLASTIC_RED_PEN_ID $RED_BOX_TOP_ID $BOX_BOTTOM_ID $BOX_INSERT_ID

PLASTIC_BLUE_PEN_PACKAGE_ID=$(add_part "plastic blue pen package")
update_part $PLASTIC_BLUE_PEN_PACKAGE_ID $PLASTIC_BLUE_PEN_ID $BLUE_BOX_TOP_ID $BOX_BOTTOM_ID $BOX_INSERT_ID

PLASTIC_BLACK_PEN_PACKAGE_ID=$(add_part "plastic black pen package")
update_part $PLASTIC_BLACK_PEN_PACKAGE_ID $PLASTIC_BLACK_PEN_ID $BLACK_BOX_TOP_ID $BOX_BOTTOM_ID $BOX_INSERT_ID

