#include "rust/cxx.h"
#include <iostream>
#include <memory>
#include <string>

#include <BRepPrimAPI_MakeBox.hxx>
#include <BRepPrimAPI_MakeCylinder.hxx>
#include <IFSelect_ReturnStatus.hxx>
#include <Interface_Static.hxx>
#include <STEPControl_Writer.hxx>
#include <TopoDS_Shape.hxx>

namespace oasm {
namespace occt {

class ShapeWrapper {
public:
  TopoDS_Shape shape;
  ShapeWrapper(TopoDS_Shape s) : shape(s) {}
};

std::unique_ptr<ShapeWrapper> make_box(double x, double y, double z) {
  TopoDS_Shape s = BRepPrimAPI_MakeBox(x, y, z).Shape();
  return std::make_unique<ShapeWrapper>(s);
}

std::unique_ptr<ShapeWrapper> make_cylinder(double r, double h) {
  TopoDS_Shape s = BRepPrimAPI_MakeCylinder(r, h).Shape();
  return std::make_unique<ShapeWrapper>(s);
}

bool export_step(const ShapeWrapper &shape, const rust::String &filename) {
  STEPControl_Writer writer;
  IFSelect_ReturnStatus status = writer.Transfer(shape.shape, STEPControl_AsIs);
  if (status != IFSelect_RetDone)
    return false;
  status = writer.Write(filename.data());
  return status == IFSelect_RetDone;
}

} // namespace occt
} // namespace oasm
