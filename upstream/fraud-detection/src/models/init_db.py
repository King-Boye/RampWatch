"""Database initialization script."""
import sys
from pathlib import Path

# Add project root to path
sys.path.insert(0, str(Path(__file__).parent.parent.parent))

from src.models.database import engine, Base, init_db
from src.models.db_models import Transaction, Features, Prediction, Alert, UserBaseline
from config.settings import settings


def create_tables():
    """Create all database tables."""
    print(f"Creating tables in database: {settings.database_url}")
    
    try:
        # Import all models to ensure they're registered
        from src.models import db_models
        
        # Create all tables
        Base.metadata.create_all(bind=engine)
        
        print("✓ Successfully created all tables:")
        for table in Base.metadata.sorted_tables:
            print(f"  - {table.name}")
            
    except Exception as e:
        print(f"✗ Error creating tables: {e}")
        raise


def drop_tables():
    """Drop all database tables (use with caution!)."""
    print(f"WARNING: Dropping all tables in database: {settings.database_url}")
    response = input("Are you sure? Type 'yes' to confirm: ")
    
    if response.lower() == 'yes':
        Base.metadata.drop_all(bind=engine)
        print("✓ All tables dropped")
    else:
        print("✗ Operation cancelled")


if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Database management")
    parser.add_argument(
        "action",
        choices=["create", "drop"],
        help="Action to perform: create or drop tables"
    )
    
    args = parser.parse_args()
    
    if args.action == "create":
        create_tables()
    elif args.action == "drop":
        drop_tables()
